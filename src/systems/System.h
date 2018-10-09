#ifndef SYSTEM_H
#define SYSTEM_H

class System
{
public:
  virtual void Initialize() = 0;
  virtual void Update() = 0;
  virtual void Destroy() = 0;
private:

};

#endif // SYSTEM_H
