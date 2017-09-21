
#include<stdio.h>
#include<cstring>
#include<string>
#include<vector>
#include<set>
#include<queue>
#define REP(i, b, n) for (int i = b; i < n; i++)
#define rep(i, n) REP(i, 0, n)
#define DBG 0
using namespace std;

struct Node{
    Node(){}
    Node(int a,int b,int c):val(a),row(b),col(c){}
    int val,row,col;
    bool operator <(Node b)const{return val<b.val||row<b.row||col<b.col;}
};
int T,bnum=13;
set<Node>st;
int bottom[]= {5,10,11,12,15,16,17,18,20,21,22,23,24};
char chess[5][10],dir[8][2];
void init(){
    rep(i,5)T<<=1,T+=1;
    T<<=1;
    rep(i,4)T<<=1,T+=1;
    rep(i,3)T<<=1; rep(i,2)T<<=1,T+=1;
    rep(i,4)T<<=1;T<<=1,T+=1;
    rep(i,5)T<<=1;
    dir[0][0]=-2,dir[0][1]=-1,
    dir[1][0]=-2,dir[1][1]=1,
    dir[2][0]=-1,dir[2][1]=-2,
    dir[3][0]=-1,dir[3][1]=2,
    dir[4][0]=1,dir[4][1]=-2,
    dir[5][0]=1,dir[5][1]=2,
    dir[6][0]=2,dir[6][1]=-1,
    dir[7][0]=2,dir[7][1]=1;
}
void getHole(int &a,int &b){
    rep(i,5)rep(j,5)if(chess[i][j]==' ')a=i,b=j;
}
bool getH(int a,int b){
    return (a>>24-b)%2;
}
int setH(int a,int pos,int b){
    if(b)a|=1<<(24-pos);
    else a-=(a & (1<<(24-pos)) );
    return a;
}
int count(int c){
    int a=0;
    rep(i,bnum){if(getH(c,bottom[i]))a++;}
    if(DBG)printf("cnt: %d\n",a);
    return a;
}
int getC(){
    int c=0;
    rep(i,5)rep(j,5){
        if(chess[i][j]=='1')c<<=1,c+=1;
        else c<<=1;
    }
    return c;
}
int move(int v,int cr,int cc,int nr,int nc){
    int from=cr*5+cc,to=nr*5+nc;
    int h=getH(v,from);
    v=setH(v,from,0);
    v=setH(v,to,h);         //move
    return v;
}
bool valid(int a,int b){return a>=0&&a<5&&b>=0&&b<5;}
void print(int a){
    rep(i,5){
        rep(j,5)printf("%d",getH(a,i*5+j));
        printf("\n");
    }
}
void ans(){
    int cur,dis,next,cr,cc,nr,nc;
    bool h;
    Node nd;
    int res=11;
    st.clear();
    getHole(cr,cc);
    queue<pair<Node,int> >q;
    nd=Node(getC(),cr,cc);
    st.insert(nd);
    q.push(make_pair(nd,0));
    while(q.size()){
        cur=q.front().first.val,dis=q.front().second,
        cr=q.front().first.row,cc=q.front().first.col;
        if(DBG){printf("cur\n");print(cur);printf("cur cr cc dis %d %d %d %d\n",cur,cr,cc,dis);}
        q.pop();
        if(dis>10)break;
        if(count(cur)>10-dis)continue;
        if(cur==T&&cr==2&&cc==2){res=dis;break;}
        rep(i,8){
            nr=cr+dir[i][0],nc=cc+dir[i][1];
            if(valid(nr,nc)){
                if(DBG)printf("new hole %d %d\n",nr,nc);
                next=move(cur,nr,nc,cr,cc);
                nd=Node(next,nr,nc);
                if(!st.count(nd))
                    st.insert(nd),q.push(make_pair(Node(next,nr,nc),dis+1));
            }
        }
    }
    if(res>10)printf("Unsolvable in less than 11 move(s).\n");
    else printf("Solvable in %d move(s).\n",res);
}
int main(){
    int n;
    scanf("%d ",&n);
    init();
    rep(i,n){
        rep(i,5)fgets(chess[i],10,stdin);
        ans();
    }
}