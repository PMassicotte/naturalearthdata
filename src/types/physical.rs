use crate::types::traits::Downloadable;
// use crate::PhysicalType;
use crate::Scale;

#[allow(missing_docs)]
pub enum PhysicalType {
    AntarcticIceShelvesLines,
    AntarcticIceShelvesPolys,
    BathymetryA10000,
    BathymetryAll,
    BathymetryB9000,
    BathymetryC8000,
    BathymetryD7000,
    BathymetryE6000,
    BathymetryF5000,
    BathymetryG4000,
    BathymetryH3000,
    BathymetryI2000,
    BathymetryJ1000,
    BathymetryK200,
    BathymetryL0,
    Coastline,
    GeographicLines,
    GeographyMarinePolys,
    GeographyRegionsElevationPoints,
    GeographyRegionsPoints,
    GeographyRegionsPolys,
    GlaciatedAreas,
    Graticules1,
    Graticules10,
    Graticules15,
    Graticules20,
    Graticules30,
    Graticules5,
    GraticulesAll,
    Lakes,
    LakesAustralia,
    LakesEurope,
    LakesHistoric,
    LakesNorthAmerica,
    LakesPluvial,
    Land,
    LandOceanLabelPoints,
    LandOceanSeams,
    LandScaleRank,
    MinorIslands,
    MinorIslandsCoastline,
    MinorIslandsLabelPoints,
    Ocean,
    OceanScaleRank,
    Physical,
    PhysicalBuildingBlocksAll,
    Playas,
    Reefs,
    RiversAustralia,
    RiversEurope,
    RiversLakeCenterlines,
    RiversLakeCenterlinesScaleRank,
    RiversNorthAmerica,
    Wgs84BoundingBox,
}
impl Downloadable for PhysicalType {
    fn get_url(&self, scale: &Scale) -> String {
        let scale_suffix = scale.scale_label();

        let prefix = "/vsizip/vsicurl/https://naciscdn.org/naturalearth/";

        match self {
            PhysicalType::AntarcticIceShelvesLines => format!(
                "{}{}/physical/ne_{}_antarctic_ice_shelves_lines.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::AntarcticIceShelvesPolys => format!(
                "{}{}/physical/ne_{}_antarctic_ice_shelves_polys.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryA10000 => format!(
                "{}{}/physical/ne_{}_bathymetry_A_10000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryAll => format!(
                "{}{}/physical/ne_{}_bathymetry_all.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryB9000 => format!(
                "{}{}/physical/ne_{}_bathymetry_B_9000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryC8000 => format!(
                "{}{}/physical/ne_{}_bathymetry_C_8000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryD7000 => format!(
                "{}{}/physical/ne_{}_bathymetry_D_7000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryE6000 => format!(
                "{}{}/physical/ne_{}_bathymetry_E_6000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryF5000 => format!(
                "{}{}/physical/ne_{}_bathymetry_F_5000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryG4000 => format!(
                "{}{}/physical/ne_{}_bathymetry_G_4000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryH3000 => format!(
                "{}{}/physical/ne_{}_bathymetry_H_3000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryI2000 => format!(
                "{}{}/physical/ne_{}_bathymetry_I_2000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryJ1000 => format!(
                "{}{}/physical/ne_{}_bathymetry_J_1000.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryK200 => format!(
                "{}{}/physical/ne_{}_bathymetry_K_200.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::BathymetryL0 => format!(
                "{}{}/physical/ne_{}_bathymetry_L_0.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Coastline => format!(
                "{}{}/physical/ne_{}_coastline.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GeographicLines => format!(
                "{}{}/physical/ne_{}_geographic_lines.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GeographyMarinePolys => format!(
                "{}{}/physical/ne_{}_geography_marine_polys.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GeographyRegionsElevationPoints => format!(
                "{}{}/physical/ne_{}_geography_regions_elevation_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GeographyRegionsPoints => format!(
                "{}{}/physical/ne_{}_geography_regions_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GeographyRegionsPolys => format!(
                "{}{}/physical/ne_{}_geography_regions_polys.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GlaciatedAreas => format!(
                "{}{}/physical/ne_{}_glaciated_areas.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules1 => format!(
                "{}{}/physical/ne_{}_graticules_1.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules10 => format!(
                "{}{}/physical/ne_{}_graticules_10.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules15 => format!(
                "{}{}/physical/ne_{}_graticules_15.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules20 => format!(
                "{}{}/physical/ne_{}_graticules_20.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules30 => format!(
                "{}{}/physical/ne_{}_graticules_30.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Graticules5 => format!(
                "{}{}/physical/ne_{}_graticules_5.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::GraticulesAll => format!(
                "{}{}/physical/ne_{}_graticules_all.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Lakes => format!(
                "{}{}/physical/ne_{}_lakes.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LakesAustralia => format!(
                "{}{}/physical/ne_{}_lakes_australia.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LakesEurope => format!(
                "{}{}/physical/ne_{}_lakes_europe.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LakesHistoric => format!(
                "{}{}/physical/ne_{}_lakes_historic.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LakesNorthAmerica => format!(
                "{}{}/physical/ne_{}_lakes_north_america.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LakesPluvial => format!(
                "{}{}/physical/ne_{}_lakes_pluvial.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Land => format!(
                "{}{}/physical/ne_{}_land.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LandOceanLabelPoints => format!(
                "{}{}/physical/ne_{}_land_ocean_label_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LandOceanSeams => format!(
                "{}{}/physical/ne_{}_land_ocean_seams.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::LandScaleRank => format!(
                "{}{}/physical/ne_{}_land_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::MinorIslands => format!(
                "{}{}/physical/ne_{}_minor_islands.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::MinorIslandsCoastline => format!(
                "{}{}/physical/ne_{}_minor_islands_coastline.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::MinorIslandsLabelPoints => format!(
                "{}{}/physical/ne_{}_minor_islands_label_points.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Ocean => format!(
                "{}{}/physical/ne_{}_ocean.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::OceanScaleRank => format!(
                "{}{}/physical/ne_{}_ocean_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Physical => format!(
                "{}{}/physical/{}_physical.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::PhysicalBuildingBlocksAll => format!(
                "{}{}/physical/ne_{}_physical_building_blocks_all.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Playas => format!(
                "{}{}/physical/ne_{}_playas.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Reefs => format!(
                "{}{}/physical/ne_{}_reefs.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::RiversAustralia => format!(
                "{}{}/physical/ne_{}_rivers_australia.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::RiversEurope => format!(
                "{}{}/physical/ne_{}_rivers_europe.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::RiversLakeCenterlines => format!(
                "{}{}/physical/ne_{}_rivers_lake_centerlines.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::RiversLakeCenterlinesScaleRank => format!(
                "{}{}/physical/ne_{}_rivers_lake_centerlines_scale_rank.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::RiversNorthAmerica => format!(
                "{}{}/physical/ne_{}_rivers_north_america.zip",
                prefix, scale_suffix, scale_suffix
            ),
            PhysicalType::Wgs84BoundingBox => format!(
                "{}{}/physical/ne_{}_wgs84_bounding_box.zip",
                prefix, scale_suffix, scale_suffix
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scale;

    #[test]
    fn test_get_url() {
        let scale_small = Scale::Small;
        let scale_medium = Scale::Medium;
        let scale_large = Scale::Large;

        let test_cases = vec![
            (PhysicalType::AntarcticIceShelvesLines, "10m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/10m/physical/ne_10m_antarctic_ice_shelves_lines.zip"),
            (PhysicalType::AntarcticIceShelvesPolys, "50m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/50m/physical/ne_50m_antarctic_ice_shelves_polys.zip"),
            (PhysicalType::BathymetryA10000, "110m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/110m/physical/ne_110m_bathymetry_A_10000.zip"),
            (PhysicalType::Coastline, "10m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/10m/physical/ne_10m_coastline.zip"),
            (PhysicalType::Lakes, "50m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/50m/physical/ne_50m_lakes.zip"),
            (PhysicalType::RiversNorthAmerica, "110m", "/vsizip/vsicurl/https://naciscdn.org/naturalearth/110m/physical/ne_110m_rivers_north_america.zip"),
        ];

        for (physical_type, scale_suffix, expected_url) in test_cases {
            let scale = match scale_suffix {
                "10m" => &scale_small,
                "50m" => &scale_medium,
                "110m" => &scale_large,
                _ => panic!("Unexpected scale suffix"),
            };

            let url = physical_type.get_url(scale);
            assert_eq!(url, expected_url);
        }
    }
}
