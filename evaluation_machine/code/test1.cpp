#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {
    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    printf("----------check----------\n");
    command = "tree -a " + path + ".mygit";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    return 0;
}
