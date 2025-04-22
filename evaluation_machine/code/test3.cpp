#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {

    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "branch " + "test " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "checkout " + "test " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    printf("----------check----------\n");
    command = "tree -a " + path + ".mygit";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "branch " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    return 0;
}
