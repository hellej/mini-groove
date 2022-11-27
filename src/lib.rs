use geojson::FeatureCollection;
use pyo3::prelude::*;
use wkt::ToWkt;

mod make_fc_from_polygons;
mod read_polygons_from_fc;
mod read_polygons_from_wkt;
mod smooth_simplify_polygons;

pub fn smooth_and_simplify_polygons_from_wkts_impl(
    wkts: &[&str],
    simplify_tolerance_m: &f64,
) -> Vec<String> {
    smooth_simplify_polygons::smooth_simplify_polygons(
        read_polygons_from_wkt::read_polygons_from_wkts(wkts),
        simplify_tolerance_m,
    )
    .iter()
    .map(|p| p.wkt_string())
    .collect()
}

#[pyfunction]
fn smooth_and_simplify_polygons_from_wkts(
    polygon_wkts: Vec<&str>,
    simplify_tolerance_m: f64,
) -> PyResult<Vec<String>> {
    Ok(smooth_and_simplify_polygons_from_wkts_impl(
        &polygon_wkts,
        &simplify_tolerance_m,
    ))
}

pub fn smooth_and_simplify_polygon_fc_impl(
    geojson: &str,
    simplify_tolerance_m: &f64,
) -> FeatureCollection {
    make_fc_from_polygons::make_fc_from_polygons(
        smooth_simplify_polygons::smooth_simplify_polygons(
            read_polygons_from_fc::read_polygons_from_feature_collection(geojson)
                .expect("Empty or invalid GeoJSON"),
            simplify_tolerance_m,
        ),
    )
}

#[pyfunction]
fn smooth_and_simplify_polygon_fc(geojson: &str, simplify_tolerance_m: f64) -> PyResult<String> {
    Ok(smooth_and_simplify_polygon_fc_impl(geojson, &simplify_tolerance_m).to_string())
}

#[pymodule]
fn mini_groove(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(smooth_and_simplify_polygon_fc, m)?)?;
    m.add_function(wrap_pyfunction!(smooth_and_simplify_polygons_from_wkts, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_data;

    #[test]
    fn test_smooth_and_simplify_polygon_fc() {
        let result =
            smooth_and_simplify_polygon_fc_impl(test_data::VECTORIZED_RASTER_POLYGON_FC, &0.3)
                .to_string();
        assert!(result.contains("Feature"));
        assert!(result.contains("FeatureCollection"));
    }
}
