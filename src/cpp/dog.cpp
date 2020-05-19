/*
    Dog.cpp
    Reference: https://qiita.com/moriai/items/e8e8b9c6a12f5a529d85
*/

#include "dog.hpp"
#include <cstdio>

Dog::Dog(const char *name) : _name(name), _status(STATUS_STOP)
{
    std::printf("%s: Good morning, bow!\n", _name);
}

Dog::~Dog()
{
    std::printf("%s: Good night, bow.\n", _name);
}

void Dog::walk()
{
    std::printf("%s: Bow bow!\n", _name);
    if (_status != STATUS_WARKING)
        _status = STATUS_WARKING;
}

void Dog::stop()
{
    std::printf("%s: Bow.\n", _name);
    if (_status != STATUS_STOP)
        _status = STATUS_STOP;
}
