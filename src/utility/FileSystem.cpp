#include "FileSystem.h"

#include <iostream>
#include <fstream>
#include <cstdio>
#include <string>
#include <algorithm>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "../utility/Logging.h"

using namespace std;

FileSystem::FileSystem(Input& input, Time& time, Map& map)
  : input_{input}
  , time_{time}
  , map_{map}
  , users_{}
{
}


void FileSystem::init()
{
  cout << "FileSystem initializing" << endl;

  // create_user("test user");

  // save_state("michael1");
  // load_state("michael1");
  // delete_state("michael1");

  // save_map("test_map1");
  // load_map("test_map1");
  // delete_map("test_map1");
}


bool FileSystem::create_user(const string& username)
{
  bool user_exists{false};

  for (const auto& user : users_)
    if (user.username == username) user_exists = true;

  if (user_exists) {
    cerr << "User already exists: " << username << endl;
    return false;
  } else {
    User user;
    user.username = username;
    users_.push_back(user);

    cout << "User created: " << username << endl;

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
    cerr << "User does not exist: " << username << endl;
    return false;
  } else {
    users_.erase(users_.begin() + index);
    cout << "User deleted: " << username << endl;
    return true;
  }
}


bool FileSystem::save_state(const string& filename)
{
  ofstream ofs("saves/" + filename);

  if (ofs.fail()) {
    cerr << "Save error: " << strerror(errno) << endl;
    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << users_;
    oa << time_;

    cout << "Save: " << filename << endl;

    return true;
  }
}


bool FileSystem::load_state(const string& filename)
{
  ifstream ifs("saves/" + filename);

  if (ifs.fail()) {
    cerr << "Load error: " << strerror(errno) << endl;
    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> users_;
    ia >> time_;

    cout << "Load: " << filename << endl;

    return true;
  }
}


bool FileSystem::delete_state(const string& filename)
{
  string filepath{"saves/" + filename};

  if (remove(filepath.c_str()) != 0) {
    cerr << "Delete error: " << strerror(errno) << endl;
    return false;
  } else {
    cout << "Delete: " << filename << endl;
    return true;
  }
}


bool FileSystem::save_map(const string& filename)
{
  ofstream ofs("maps/" + filename);

  if (ofs.fail()) {
    cerr << "Map save error: " << strerror(errno) << endl;
    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << map_;

    cout << "Map save: " << filename << endl;

    return true;
  }
}


bool FileSystem::load_map(const string& filename)
{
  ifstream ifs("maps/" + filename);

  if (ifs.fail()) {
    cerr << "Load map error: " << strerror(errno) << endl;
    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> map_;

    cout << "Load map: " << filename << endl;

    return true;
  }
}


bool FileSystem::delete_map(const string& filename)
{
  string filepath{"maps/" + filename};

  if (remove(filepath.c_str()) != 0) {
    cerr << "Delete map error: " << strerror(errno) << endl;
    return false;
  } else {
    cout << "Delete map: " << filename << endl;

    return true;
  }
}

