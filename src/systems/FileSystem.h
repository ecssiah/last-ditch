#ifndef FILE_SYSTEM_H
#define FILE_SYSTEM_H

#include "../components/Input.h"
#include "../components/User.h"
#include "../components/map/Map.h"

class FileSystem
{
public:
  FileSystem(Input& input, Map& map);

  void Initialize();

  bool CreateUser(std::string username);
  bool DeleteUser(std::string username);

  bool Save(std::string filename);
  bool Load(std::string filename);

  bool SaveMap(std::string filename);
  bool LoadMap(std::string filename);

private:
  Input& input_;
  Map& map_;

  std::vector<User> users_;

};

#endif
