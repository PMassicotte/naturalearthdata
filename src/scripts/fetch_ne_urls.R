library(tidyverse)
library(rvest)
library(glue)
library(curl)
library(httr2)
library(fs)
library(janitor)

get_raw_links <- function(base_url) {
  urls <- read_html(base_url) |>
    html_elements(".download-link") |>
    html_attr("href")

  urls
}

fix_urls <- function(urls) {
  str_replace_all(
    urls,
    "https://www.naturalearthdata.com/http//www.naturalearthdata.com/download/",
    "https://naciscdn.org/naturalearth/"
  )
}

base_page <- "https://www.naturalearthdata.com/downloads/{scale}-{type}-vectors/"

# Create a list of download links
urls <- tibble(scale = c("10m", "50m", "110m")) |>
  crossing(type = c("cultural", "physical")) |>
  mutate(urls = glue(base_page, scale = scale, type = type)) |>
  mutate(raw_links = map(urls, get_raw_links)) |>
  mutate(fixed_urls = map(raw_links, fix_urls))

urls <- urls |>
  unnest(fixed_urls) |>
  select(scale, type, fixed_urls) |>
  mutate(url_ok = RCurl::url.exists(fixed_urls)) |>
  filter(url_ok) |>
  mutate(fixed_urls = paste0("/vsizip/vsicurl/", fixed_urls)) |>
  select(-url_ok)

urls

# Create the enum name for rust

urls <- urls |>
  mutate(basename = path_file(fixed_urls), .before = fixed_urls) |>
  mutate(basename = path_ext_remove(basename)) |>
  mutate(enum_name = str_remove(basename, ".*\\d+m_"), .after = basename) |>
  mutate(
    enum_name = janitor::make_clean_names(
      enum_name,
      case = "upper_camel",
      allow_dupes = TRUE
    )
  )

# CulturalType::Admin0Countries => format!("/vsizip/vsicurl/https://naciscdn.org/naturalearth/{}/cultural/ne_{}_admin_0_countries.zip", scale_suffix, scale_suffix),

rust_match <- r"({type}Type::{enum_name} => format!("{fixed_urls}", prefix, scale_suffix, scale_suffix),)"

urls

# %% ---- Cultural links
urls |>
  filter(type == "cultural") |>
  arrange(enum_name) |>
  mutate(fixed_urls = str_replace_all(fixed_urls, "\\d+m", "{}")) |>
  mutate(rust_match_line = glue(rust_match, type = str_to_title(type))) |>
  distinct(fixed_urls, .keep_all = TRUE) |>
  mutate(
    rust_match_line = str_replace_all(
      rust_match_line,
      "/vsizip/vsicurl/https://naciscdn.org/naturalearth/",
      "{}"
    )
  ) |>
  pull(rust_match_line) |>
  cat()

urls |>
  filter(type == "cultural") |>
  select(enum_name) |>
  distinct() |>
  arrange(enum_name) |>
  mutate(enum_name = paste0(enum_name, ",")) |>
  pull(enum_name) |>
  cat()
# %%

# %% ---- Physical links
urls |>
  filter(type == "physical") |>
  arrange(enum_name) |>
  mutate(fixed_urls = str_replace_all(fixed_urls, "\\d+m", "{}")) |>
  mutate(rust_match_line = glue(rust_match, type = str_to_title(type))) |>
  distinct(fixed_urls, .keep_all = TRUE) |>
  mutate(
    rust_match_line = str_replace_all(
      rust_match_line,
      "/vsizip/vsicurl/https://naciscdn.org/naturalearth/",
      "{}"
    )
  ) |>
  pull(rust_match_line) |>
  cat()

urls |>
  filter(type == "physical") |>
  select(enum_name) |>
  distinct() |>
  arrange(enum_name) |>
  mutate(enum_name = paste0(enum_name, ",")) |>
  pull(enum_name) |>
  cat()
# %%
