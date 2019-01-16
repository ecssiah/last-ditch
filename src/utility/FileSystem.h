#ifndef FILE_SYSTEM_H
#define FILE_SYSTEM_H

#include "Types.h"
#include "User.h"
#include "../map/Map.h"
#include "../time/Time.h"
#include "../interface/Input.h"

class FileSystem
{
public:
  FileSystem(Input& input, Time& time, Map& map);

  void init();

  bool create_user(const std::string& username);
  bool delete_user(const std::string& username);

  bool save_state(const std::string& filename);
  bool load_state(const std::string& filename);
  bool delete_state(const std::string& filename);

  bool save_map(const std::string& filename);
  bool load_map(const std::string& filename);
  bool delete_map(const std::string& filename);

private:
  Input& input_;
  Time& time_;
  Map& map_;

  std::vector<User> users_;

};

#endif
