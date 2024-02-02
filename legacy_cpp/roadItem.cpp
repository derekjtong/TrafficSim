#include <iostream>
#include <string>

struct Point {
    double x;
    double y;
};

class RoadItem {
    Point pos;

   public:
    void setPos(double x, double y);
    int getX();
    int getY();
};

class DynamicRoadItem : RoadItem {
   public:
    DynamicRoadItem();
    DynamicRoadItem(double x, double y);
    virtual ~DynamicRoadItem();
    void moveUp();
    void moveDown();
    void moveLeft();
    void moveRight();
};

class StaticRoadItem : RoadItem {
   public:
    StaticRoadItem();
    StaticRoadItem(double x, double y);
    virtual ~StaticRoadItem();
};

int main() {
    std::cout << "hello" << std::endl;
}