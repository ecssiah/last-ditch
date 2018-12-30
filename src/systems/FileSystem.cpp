#include "FileSystem.h"

#include <iostream>
#include <fstream>
#include <cstdio>
#include <string>
#include <algorithm>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

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
  create_user("test user");

  save_state("michael1");
  load_state("michael1");
  delete_state("michael1");

  save_map("test_map1");
  load_map("test_map1");
  delete_map("test_map1");
}


bool FileSystem::create_user(const string& username)
{
  bool user_exists{false};

  for (const auto& user : users_) {
    if (user.username == username) user_exists = true;
  } 

  if (user_exists) {
    cerr << "User exists: " << username << endl;

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
  i32 index;
  bool user_exists{false};

  for (auto i{0}; i < users_.size(); ++i) {
    if (users_[i].username == username) {
      index = i;
      user_exists = true;
    }
  }

  if (user_exists) {
    users_.erase(users_.begin() + index);

    cout << "User erased: " << username << endl;

    return true;
  } else {
    cerr << "User does not exist: " << username << endl;

    return false;
  }
}


bool FileSystem::save_state(const string& filename)
{
  ofstream ofs("saves/" + filename);

  if (ofs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << users_;
    oa << time_;

    cout << "Saved: " << filename << endl;

    return true;
  }
}


bool FileSystem::load_state(const string& filename)
{
  ifstream ifs("saves/" + filename);

  if (ifs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> users_;
    ia >> time_;

    cout << "Loaded: " << filename << endl;

    return true;
  }
}


bool FileSystem::delete_state(const string& filename)
{
  string filepath{"saves/" + filename};

  if (remove(filepath.c_str()) == 0) {
    cout << "Save deleted: " << filename << endl;
    return true;
  } else {
    cerr << "Error " << errno << ":" << filename << " was not deleted" << endl;
    return false;
  }
}


bool FileSystem::save_map(const string& filename)
{
  ofstream ofs("maps/" + filename);

  if (ofs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << map_;

    cout << "Map saved : " << filename << endl;

    return true;
  }
}


bool FileSystem::load_map(const string& filename)
{
  ifstream ifs("maps/" + filename);

  if (ifs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> map_;

    cout << "Map loaded: " << filename << endl;

    return true;
  }
}


bool FileSystem::delete_map(const string& filename)
{
  string filepath{"maps/" + filename};

  if (remove(filepath.c_str()) == 0) {
    cout << "Map deleted: " << filename << endl;
    return true;
  } else {
    cerr << "Error " << errno << ": " << filename << " was not deleted" << endl;
    return false;
  }
}

