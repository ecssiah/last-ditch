#ifndef FILE_SYSTEM_H
#define FILE_SYSTEM_H

#include "../components/Input.h"
#include "../components/User.h"
#include "../components/Time.h"
#include "../components/map/Map.h"

class FileSystem
{
public:
  FileSystem(Input& input, Map& map, Time& time);

  void Initialize();

  bool CreateUser(const std::string& username);
  bool DeleteUser(const std::string& username);

  bool Save(const std::string& filename);
  bool Load(const std::string& filename);
  bool Delete(const std::string& filename);

  bool SaveMap(const std::string& filename);
  bool LoadMap(const std::string& filename);
  bool DeleteMap(const std::string& filename);

private:
  Input& input_;
  Map& map_;
  Time& time_;

  std::vector<User> users_;

};

#endif
