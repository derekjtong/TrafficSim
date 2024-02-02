#include <vector>

#include "roadItem.cpp"

class Road {
    std::vector<RoadItem> roadItems;

   public:
    Road();
    Road(std::vector<RoadItem> roadItems);
    ~Road();
    void addRoadItem(RoadItem roadItem);
    void removeRoadItem(RoadItem roadItem);
    std::vector<RoadItem> getRoadItems();
};