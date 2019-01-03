#include "../../include/utility/FileSystem.h"

#include <iostream>
#include <fstream>
#include <cstdio>
#include <string>
#include <algorithm>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "../../include/utility/Logging.h"

using namespace std;

FileSystem::FileSystem(Input& input, Map& map, Time& time)
  : input_{input}
  , map_{map}
  , time_{time}
  , users_{}
{
}


void FileSystem::init()
{
  log("FileSystem initializing");

//   create_user("test user");

//   save_state("michael1");
//   load_state("michael1");
//   delete_state("michael1");

//   save_map("test_map1");
//   load_map("test_map1");
//   delete_map("test_map1");
}


bool FileSystem::create_user(const string& username)
{
  bool user_exists{false};

  for (const auto& user : users_) {
    if (user.username == username) user_exists = true;
  } 

  if (user_exists) {
    elog("User already exists: " + username);
    return false;
  } else {
    User user;
    user.username = username;
    users_.push_back(user);

    log("User created: " + username);

    return true;
  }
}


bool FileSystem::delete_user(const string& username)
{
  i32 index{0};
  bool user_exists{false};

  for (auto i{0}; i < users_.size(); ++i) {
    if (users_[i].username == username) {
      index = i;
      user_exists = true;
    }
  }

  if (!user_exists) {
    elog("User does not exist: " + username);
    return false;
  } else {
    users_.erase(users_.begin() + index);
    log("User deleted: " + username);
    return true;
  }
}


bool FileSystem::save_state(const string& filename)
{
  ofstream ofs("saves/" + filename);

  if (ofs.fail()) {
    elog("Save error: " + string(strerror(errno)));
    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << users_;
    oa << time_;

    log("Save: " + filename);

    return true;
  }
}


bool FileSystem::load_state(const string& filename)
{
  ifstream ifs("saves/" + filename);

  if (ifs.fail()) {
    elog("Load error: " + string(strerror(errno)));
    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> users_;
    ia >> time_;

    log("Load: " + filename);

    return true;
  }
}


bool FileSystem::delete_state(const string& filename)
{
  string filepath{"saves/" + filename};

  if (remove(filepath.c_str()) != 0) {
    elog("Delete error: " + string(strerror(errno)));
    return false;
  } else {
    log("Delete: " + filename);
    return true;
  }
}


bool FileSystem::save_map(const string& filename)
{
  ofstream ofs("maps/" + filename);

  if (ofs.fail()) {
    elog("Map save error: " + string(strerror(errno)));
    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << map_;

    log("Map save: " + filename);

    return true;
  }
}


bool FileSystem::load_map(const string& filename)
{
  ifstream ifs("maps/" + filename);

  if (ifs.fail()) {
    elog("Load map error: " + string(strerror(errno)));
    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> map_;

    log("Load map: " + filename);

    return true;
  }
}


bool FileSystem::delete_map(const string& filename)
{
  string filepath{"maps/" + filename};

  if (remove(filepath.c_str()) != 0) {
    elog("Delete map error: " + string(strerror(errno)));
    return false;
  } else {
    log("Delete map: " + filename);

    return true;
  }
}

