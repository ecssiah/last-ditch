#include "FileSystem.h"

#include <iostream>
#include <fstream>
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


void FileSystem::Initialize()
{
  CreateUser("test user");

  /* Save("michael1"); */
  /* Load("michael1"); */

  /* SaveMap("test_map1"); */
  /* LoadMap("test_map1"); */
}


bool FileSystem::CreateUser(string username)
{
  bool user_exists{false};

  for (const auto& user : users_) {
    if (user.username == username) user_exists = true;
  } 

  if (user_exists) {
    cerr << "User: " << username << " already exists" << endl;

    return false;
  } else {
    User user;
    user.username = username;

    users_.push_back(user);

    cout << "User: " << username << " created" << endl;

    return true;
  }
}


bool FileSystem::DeleteUser(string username)
{
  int index;
  bool user_exists{false};

  for (auto i{0}; i < users_.size(); ++i) {
    if (users_[i].username == username) {
      index = i;
      user_exists = true;
    }
  }

  if (user_exists) {
    users_.erase(users_.begin() + index);

    cout << "User: " << username << " erased" << endl;

    return true;
  } else {
    cerr << "User: " << username << " does not exist" << endl;

    return false;
  }
}


bool FileSystem::Save(string filename)
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


bool FileSystem::Load(string filename)
{
  ifstream ifs("saves/" + filename);

  if (ifs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> users_;
    ia >> time_;

    cout << "Saved: " << filename << endl;

    return true;
  }
}


bool FileSystem::SaveMap(string filename)
{
  ofstream ofs("maps/" + filename);

  if (ofs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << map_;

    cout << "Map saved as: " << filename << endl;

    return true;
  }
}


bool FileSystem::LoadMap(string filename)
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

