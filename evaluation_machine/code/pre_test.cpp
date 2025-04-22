#include<bits/stdc++.h>
using namespace std;
string path = "./test_area/";
string command{};
int main(){
    command = "rm -rf " + path + "*";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    command = "rm -rf " + path + ".mygit";
    cout<<">run:"<<command<<endl;
    system(command.c_str());
    return 0;
}