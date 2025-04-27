#include<bits/stdc++.h>
using namespace std;
string git = "../test/git ";
string path = "./test_area/";
string command{};
int main() {
    command = git + "init -p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "branch " + "main " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "checkout " + "main " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());

    command = "touch " + path + "main.rs";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "cat ./code/main.rs > " + path + "main.rs";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "add " + path + "main.rs " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "commit " + "-m \"main\" " + "-p " + path;
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

    command = "touch " + path + "test.txt";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "echo \"测试分支合并\" > " + path + "test.txt";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "add " + path + "test.txt " + "-p " + path;
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = git + "commit " + "-m \"test\" " + "-p " + path;
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
    command = "rustc " + path + "main.rs -o " + path + "main";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = path + "main";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    return 0;
}
