from typing import List


def smooth_and_simplify_polygon_fc(geojson: str, simplify_tolerance_m: float) -> str:
    """
    Reads polygons from GeoJSON FeatureCollection, smoothens and simplifies them.
    The FeatureCollection must contain only Polygons (and/or MultiPolygons). 
    Expects the coordinates of the features to be in meters.
    
    Returns GeoJSON FeatureCollection as string.
    """
    pass


def smooth_and_simplify_polygons_from_wkts(
    polygon_wkts: List[str],
    simplify_tolerance_m: float
) -> List[str]:
    """Reads polygons from list of WKT Polygons, smoothens and simplifies them.
    Supports only WKT Polygons. Expects the coordinates of the geometries to be in meters.

    Returns simplified geometries as list of WKTs.
    """
    pass