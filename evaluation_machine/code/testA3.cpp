#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {

    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "branch " + "temp " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "checkout " + "temp " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "branch -d " + "temp " + "-p " + path;
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
