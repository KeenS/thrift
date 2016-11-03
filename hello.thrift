namespace rust thrift
service Flock {
  bool isLoggedIn(1: string token);
  bool isLoggedOut(1: string token);
}
