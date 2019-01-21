#include "EventSystem.h"

#include <iostream>
#include <string>

#include "../../ext/simple_signal/SimpleSignal.h"

using namespace std;


struct Tester
{
  int test_func(int a, string s)
  {
    cout << s << endl;

    return a;
  }

};



EventSystem::EventSystem()
{

}


void
EventSystem::init()
{
  Simple::Signal<int (int, string)> sig;

  Tester tester;
  sig.connect(Simple::slot(tester, &Tester::test_func));
  cout << sig.emit(2, "test") << endl;

}


void
EventSystem::update()
{


}