#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {
    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "touch " + path + "test.txt";
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = git + "branch " + "main " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "checkout " + "main " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "echo \"main分支修改内容\" > " + path + "test.txt";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "add " + path + "test.txt " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "commit " + "-m \"main1\" " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = git + "checkout " + "master " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = git + "branch " + "test " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "checkout " + "test " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "echo \"test分支修改内容\" > " + path + "test.txt";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "add " + path + "test.txt " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "commit " + "-m \"test1\" " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = git + "checkout " + "main " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "merge " + "test " + "-p " + path;
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
}
