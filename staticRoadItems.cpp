#include "roadItem.cpp"

class StopSign : public StaticRoadItem {
};

class Intersection : public StaticRoadItem {
};

class SpeedLimitSign : public StaticRoadItem {
    int speedLimit;

   public:
    SpeedLimitSign(int speedLimit);
    int getSpeedLimit();
    void setSpeedLimit(int speedLimit);
};

class YieldSign : public StaticRoadItem {
};