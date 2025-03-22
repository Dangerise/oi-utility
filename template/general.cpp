#include <bits/stdc++.h>
using namespace std;

#define int long long
// #define double long double
#define siz(x) ((int)(x).size())
#define me(a, v) memset(a, v, sizeof(a))
#define cp(a, b) memcpy(b, a, (assert(sizeof(a) == sizeof(b)), sizeof(a)))
#define L(i, l, r) for (int i = (l); i <= (r); i++)
#define R(i, r, l) for (int i = (r); i >= (l); i--)
#define x first
#define y second

#define ve(a) vector<a>
#define vep(a, b) vector<pair<a, b>>
#define p(a, b) pair<a, b>
#define map(a, b) map<a, b>
#define set(a) set<a>
#define mmap(a, b) multimap<a, b>
#define mset(a) multiset<a>
#define hmap(a, b) unordered_map<a, b>
#define hset(a) unordered_set<a>
#define bs(n) bitset<n>

typedef double flt;
typedef string str;
typedef signed sint;
typedef __int128 bint;
typedef p(int, int) pii;

char gc() { return getchar(); }
void pc(char c) { putchar(c); }
void el() { pc('\n'); }
void bp() {
    puts("Fuck ccf !");
    fflush(stdout);
}
void dtm(bool cond) { puts(cond ? "Yes" : "No"); }
void spc() { pc(' '); }
// il __int128 abs(__int128 x){
// 	return x<0?-x:x;
// }
char qrc() {
    auto av = [&](char c) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    };
    char c = gc();
    while (!av(c)) {
        c = gc();
    }
    return c;
}
void qrs(string &s) {
    auto av = [&](char c) {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
               (c >= '0' && c <= '9');
    };
    char c = gc();
    while (!av(c)) {
        c = gc();
    }
    s.clear();
    while (av(c)) {
        s.push_back(c);
        c = gc();
    }
}
int qread() {
    int ans = 0, f = 1;
    char c = gc();
    while (c < '0' || c > '9') {
        if (c == '-') {
            f = -1;
        }
        c = gc();
    }
    while (c >= '0' && c <= '9') {
        ans = ans * 10 + c - '0';
        c = gc();
    }
    return ans * f;
}

constexpr int mod = 1e9 + 7;
// constexpr int mod = 998244353;

void updmax(int &x, int y) { x = max(x, y); }
void updmin(int &x, int y) { x = min(x, y); }
int gcd(int x, int y) { return __gcd(x, y); }
int lcm(int x, int y) { return x / gcd(x, y) * y; }
int lowbit(int x) { return x & (-x); }
int max(int x, int y) { return x > y ? x : y; }
int min(int x, int y) { return x < y ? x : y; }
int sqr(int x) { return x * x; }
int ceil(int x, int y) { return (x + y - 1) / y; }

constexpr int N = -1;
constexpr int inf = 0x3f3f3f3f3f3f3f3f;

void solve() {}

sint main() {
    int T = qread();
    while (T--) {
        solve();
    }
    return 0;
}
