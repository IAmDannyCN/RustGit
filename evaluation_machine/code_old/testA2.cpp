#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {
    command = "touch " + path + "large_file.bin";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "cat ./code/large_file.bin > " + path + "large_file.bin";
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "add " + path + "large_file.bin " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "commit " + "-m \"Add large file\" " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    printf("----------check----------\n");
    command = "tree -a " + path + ".mygit";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "status " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "log " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    return 0;
}
