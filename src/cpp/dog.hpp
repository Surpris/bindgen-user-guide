/*
    Dog.hpp
    Refernce: https://qiita.com/moriai/items/e8e8b9c6a12f5a529d85
*/

class Dog
{
public:
    enum status
    {
        STATUS_WARKING,
        STATUS_STOP,
        STATUS_EATING
    };

private:
    const char *_name;
    enum status _status;

public:
    Dog(const char *name);
    ~Dog();
    void walk();
    void stop();
    enum status status() const { return _status; }
};
