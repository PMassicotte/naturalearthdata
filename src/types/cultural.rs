use crate::types::traits::Downloadable;
use crate::Scale;

#[allow(missing_docs)]
pub enum CulturalType {
    Admin0AntarcticClaimLimitLines,
    Admin0AntarcticClaims,
    Admin0BoundaryLinesDisputedAreas,
    Admin0BoundaryLinesLand,
    Admin0BoundaryLinesMapUnits,
    Admin0BoundaryLinesMaritimeIndicator,
    Admin0BoundaryLinesMaritimeIndicatorChn,
    Admin0BoundaryMapUnits,
    Admin0BreakawayDisputedAreas,
    Admin0Countries,
    Admin0CountriesArg,
    Admin0CountriesBdg,
    Admin0CountriesBra,
    Admin0CountriesChn,
    Admin0CountriesDeu,
    Admin0CountriesEgy,
    Admin0CountriesEsp,
    Admin0CountriesFra,
    Admin0CountriesGbr,
    Admin0CountriesGrc,
    Admin0CountriesIdn,
    Admin0CountriesInd,
    Admin0CountriesIso,
    Admin0CountriesIsr,
    Admin0CountriesIta,
    Admin0CountriesJpn,
    Admin0CountriesKor,
    Admin0CountriesLakes,
    Admin0CountriesMar,
    Admin0CountriesNep,
    Admin0CountriesNld,
    Admin0CountriesPak,
    Admin0CountriesPol,
    Admin0CountriesPrt,
    Admin0CountriesPse,
    Admin0CountriesRus,
    Admin0CountriesSau,
    Admin0CountriesSwe,
    Admin0CountriesTlc,
    Admin0CountriesTur,
    Admin0CountriesTwn,
    Admin0CountriesUkr,
    Admin0CountriesUsa,
    Admin0CountriesVnm,
    Admin0DisputedAreas,
    Admin0DisputedAreasScaleRankMinorIslands,
    Admin0LabelPoints,
    Admin0MapSubunits,
    Admin0MapUnits,
    Admin0PacificGroupings,
    Admin0ScaleRank,
    Admin0ScaleRankMinorIslands,
    Admin0Seams,
    Admin0Sovereignty,
    Admin0TinyCountries,
    Admin0TinyCountriesScaleRank,
    Admin1LabelPoints,
    Admin1LabelPointsDetails,
    Admin1Seams,
    Admin1StatesProvinces,
    Admin1StatesProvincesLakes,
    Admin1StatesProvincesLines,
    Admin1StatesProvincesScaleRank,
    Admin2Counties,
    Admin2CountiesLakes,
    Admin2CountiesScaleRank,
    Admin2CountiesScaleRankMinorIslands,
    Admin2LabelPoints,
    Admin2LabelPointsDetails,
    Airports,
    Cultural,
    CulturalBuildingBlocksAll,
    ParksAndProtectedLands,
    PopulatedPlaces,
    PopulatedPlacesSimple,
    Ports,
    Railroads,
    RailroadsNorthAmerica,
    Roads,
    RoadsNorthAmerica,
    TimeZones,
    UrbanAreas,
}

impl Downloadable for CulturalType {
    fn get_url(&self, scale: &Scale) -> String {
        let prefix = "/vsizip/vsicurl/https://naciscdn.org/naturalearth/";

        let scale_suffix = scale.suffix();

        match self {
            CulturalType::Admin0AntarcticClaimLimitLines => format!(
                "{}{}/cultural/ne_{}_admin_0_antarctic_claim_limit_lines.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0AntarcticClaims => format!(
                "{}{}/cultural/ne_{}_admin_0_antarctic_claims.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryLinesDisputedAreas => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_lines_disputed_areas.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryLinesLand => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_lines_land.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryLinesMapUnits => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_lines_map_units.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryLinesMaritimeIndicator => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_lines_maritime_indicator.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryLinesMaritimeIndicatorChn => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_lines_maritime_indicator_chn.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BoundaryMapUnits => format!(
                "{}{}/cultural/ne_{}_admin_0_boundary_map_units.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0BreakawayDisputedAreas => format!(
                "{}{}/cultural/ne_{}_admin_0_breakaway_disputed_areas.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0Countries => format!(
                "{}{}/cultural/ne_{}_admin_0_countries.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesArg => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_arg.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesBdg => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_bdg.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesBra => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_bra.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesChn => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_chn.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesDeu => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_deu.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesEgy => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_egy.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesEsp => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_esp.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesFra => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_fra.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesGbr => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_gbr.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesGrc => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_grc.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesIdn => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_idn.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesInd => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_ind.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesIso => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_iso.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesIsr => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_isr.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesIta => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_ita.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesJpn => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_jpn.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesKor => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_kor.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesLakes => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_lakes.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesMar => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_mar.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesNep => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_nep.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesNld => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_nld.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesPak => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_pak.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesPol => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_pol.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesPrt => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_prt.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesPse => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_pse.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesRus => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_rus.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesSau => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_sau.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesSwe => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_swe.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesTlc => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_tlc.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesTur => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_tur.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesTwn => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_twn.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesUkr => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_ukr.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesUsa => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_usa.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0CountriesVnm => format!(
                "{}{}/cultural/ne_{}_admin_0_countries_vnm.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0DisputedAreas => format!(
                "{}{}/cultural/ne_{}_admin_0_disputed_areas.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0DisputedAreasScaleRankMinorIslands => format!(
                "{}{}/cultural/ne_{}_admin_0_disputed_areas_scale_rank_minor_islands.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0LabelPoints => format!(
                "{}{}/cultural/ne_{}_admin_0_label_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0MapSubunits => format!(
                "{}{}/cultural/ne_{}_admin_0_map_subunits.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0MapUnits => format!(
                "{}{}/cultural/ne_{}_admin_0_map_units.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0PacificGroupings => format!(
                "{}{}/cultural/ne_{}_admin_0_pacific_groupings.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0ScaleRank => format!(
                "{}{}/cultural/ne_{}_admin_0_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0ScaleRankMinorIslands => format!(
                "{}{}/cultural/ne_{}_admin_0_scale_rank_minor_islands.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0Seams => format!(
                "{}{}/cultural/ne_{}_admin_0_seams.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0Sovereignty => format!(
                "{}{}/cultural/ne_{}_admin_0_sovereignty.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0TinyCountries => format!(
                "{}{}/cultural/ne_{}_admin_0_tiny_countries.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin0TinyCountriesScaleRank => format!(
                "{}{}/cultural/ne_{}_admin_0_tiny_countries_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1LabelPoints => format!(
                "{}{}/cultural/ne_{}_admin_1_label_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1LabelPointsDetails => format!(
                "{}{}/cultural/ne_{}_admin_1_label_points_details.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1Seams => format!(
                "{}{}/cultural/ne_{}_admin_1_seams.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1StatesProvinces => format!(
                "{}{}/cultural/ne_{}_admin_1_states_provinces.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1StatesProvincesLakes => format!(
                "{}{}/cultural/ne_{}_admin_1_states_provinces_lakes.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1StatesProvincesLines => format!(
                "{}{}/cultural/ne_{}_admin_1_states_provinces_lines.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin1StatesProvincesScaleRank => format!(
                "{}{}/cultural/ne_{}_admin_1_states_provinces_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2Counties => format!(
                "{}{}/cultural/ne_{}_admin_2_counties.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2CountiesLakes => format!(
                "{}{}/cultural/ne_{}_admin_2_counties_lakes.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2CountiesScaleRank => format!(
                "{}{}/cultural/ne_{}_admin_2_counties_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2CountiesScaleRankMinorIslands => format!(
                "{}{}/cultural/ne_{}_admin_2_counties_scale_rank_minor_islands.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2LabelPoints => format!(
                "{}{}/cultural/ne_{}_admin_2_label_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Admin2LabelPointsDetails => format!(
                "{}{}/cultural/ne_{}_admin_2_label_points_details.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Airports => format!(
                "{}{}/cultural/ne_{}_airports.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Cultural => format!(
                "{}{}/cultural/{}_cultural.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::CulturalBuildingBlocksAll => format!(
                "{}{}/cultural/ne_{}_cultural_building_blocks_all.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::ParksAndProtectedLands => format!(
                "{}{}/cultural/ne_{}_parks_and_protected_lands.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::PopulatedPlaces => format!(
                "{}{}/cultural/ne_{}_populated_places.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::PopulatedPlacesSimple => format!(
                "{}{}/cultural/ne_{}_populated_places_simple.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Ports => format!(
                "{}{}/cultural/ne_{}_ports.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Railroads => format!(
                "{}{}/cultural/ne_{}_railroads.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::RailroadsNorthAmerica => format!(
                "{}{}/cultural/ne_{}_railroads_north_america.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::Roads => format!(
                "{}{}/cultural/ne_{}_roads.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::RoadsNorthAmerica => format!(
                "{}{}/cultural/ne_{}_roads_north_america.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::TimeZones => format!(
                "{}{}/cultural/ne_{}_time_zones.zip",
                prefix, scale_suffix, scale_suffix
            ),
            CulturalType::UrbanAreas => format!(
                "{}{}/cultural/ne_{}_urban_areas.zip",
                prefix, scale_suffix, scale_suffix
            ),
        }
    }
}
