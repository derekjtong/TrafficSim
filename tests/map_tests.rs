use trafficsim::*;

#[cfg(test)]
mod map_integration {
    use trafficsim::road_items::{Point, RoadItem};

    use crate::map::Map;
    use crate::road::Road;

    // Dummy RoadItem for testing purposes
    struct TestItem;
    impl RoadItem for TestItem {
        fn set_pos(&mut self, _pos: Point) {}
        fn pos(&self) -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        fn type_name(&self) -> &'static str {
            "TestItem"
        }
    }

    #[test]
    fn test_map_new() {
        let map = Map::new();
        assert!(map.get_roads().is_empty());
    }

    #[test]
    fn test_add_road() {
        let mut map = Map::new();
        let road = Road::new("test".to_string(), 1.0, 0.0, 0.0, crate::Heading::North);
        map.add_road(road);
        assert_eq!(map.get_roads().len(), 1);
    }

    #[test]
    fn test_remove_road() {
        let mut map = Map::new();
        let road = Road::new("test".to_string(), 1.0, 0.0, 0.0, crate::Heading::North);
        map.add_road(road);
        map.remove_road(0);
        assert!(map.get_roads().is_empty());
    }

    #[test]
    fn test_remove_road_invalid_index() {
        let mut map = Map::new();
        map.remove_road(999); // Attempt to remove a road at an invalid index
        assert!(map.get_roads().is_empty()); // Ensure no panic, map remains empty
    }

    #[test]
    fn test_total_road_items() {
        let mut map = Map::new();
        let mut road1 = Road::new("test".to_string(), 1.0, 0.0, 0.0, crate::Heading::North);
        let item1 = Box::new(TestItem);
        let item2 = Box::new(TestItem);
        road1.add_road_item(item1);
        road1.add_road_item(item2);

        let mut road2 = Road::new("test".to_string(), 1.0, 0.0, 0.0, crate::Heading::North);
        let item3 = Box::new(TestItem);
        road2.add_road_item(item3);

        map.add_road(road1);
        map.add_road(road2);

        assert_eq!(map.total_road_items(), 3); // 2 items in road1 + 1 item in road2
    }
}
