#include <bits/stdc++.h>
using namespace std;

#define int long long
// #define double long double
#define siz(x) ((int)(x).size())
#define me(a, v, n) memset(a, v, sizeof(typeof(*a)) * (n + 1))
#define cp(a, b, n)                                                            \
    memcpy(b, a,                                                               \
           (assert(sizeof(a) == sizeof(b) &&                                   \
                   sizof(typeof(*a)) == sizof(typeof(*b))),                    \
            sizeof(typeof(*a)) * (n + 1)))
#define L(i, l, r) for (int i = (l); i <= (r); i++)
#define R(i, r, l) for (int i = (r); i >= (l); i--)
#define x first
#define y second

#define ve(a) vector<a>
#define p(a, b) pair<a, b>
#define map(a, b) map<a, b>
#define set(a) set<a>
#define mmap(a, b) multimap<a, b>
#define mset(a) multiset<a>
#define hmap(a, b) unordered_map<a, b>
#define hset(a) unordered_set<a>
#define bs(n) bitset<n>

typedef double flt;
typedef signed sint;
typedef __int128 bint;

char gc() { return getchar(); }
void pc(char c) { putchar(c); }
void el() { pc('\n'); }
void bp() {
    puts("Fuck ccf !");
    fflush(stdout);
}
void dtm(bool cond) { puts(cond ? "Yes" : "No"); }
void spc() { pc(' '); }
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
    int ret = 0;
    bool sgn = 0;
    char c;
    while (c < '0' || c > '9') {
        sgn |= (c == '-');
        c = gc();
    }
    while (c >= '0' && c <= '9') {
        ret = ret * 10 + c - '0';
        c = gc();
    }
    return sgn ? -ret : ret;
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

int random(int l, int r) { return (int)rand() * rand() % (r - l + 1) + l; }

signed main() {
    freopen(".in", "w", stdout);

    srand((int)time(0));

    return 0;
}
