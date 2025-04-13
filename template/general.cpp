#include <bits/stdc++.h>
using namespace std;

#define int long long
// #define double long double
#define siz(x) ((int)(x).size())
#define me(a, v) memset(a, v, sizeof(a))
#define wr printf
#define L(i, l, r) for (int i = (l); i <= (r); i++ s)
#define R(i, r, l) for (int i = (r); i >= (l); i--)
#define mp make_pair
#define x first
#define y second

typedef signed sint;
typedef __int128 bint;

#define ve(a) Ve<a>
#define sve(a) static Ve<a>
#define p(a, b) pair<a, b>
#define map(a, b) map<a, b>
#define set(a) set<a>
#define mmap(a, b) multimap<a, b>
#define mset(a) multiset<a>
#define hmap(a, b) unordered_map<a, b>
#define hset(a) unordered_set<a>
#define hmmap(a, b) unordered_multimap<a, b>
#define hmset(a) unordered_multiset<a>
#define bs(n) bitset<n>

#ifdef DGR
#define dwr printf
#define dbg(x) dwr("%s = ", #x), output(x), el()
#define ast(expr, info)                                                        \
    ((expr) ? void(0)                                                          \
            : (dwr("Assert %s failed at %d\n", #expr, __LINE__), dbg(info),    \
               dwr("\n"), throw 0))
#define rdf(p) freopen(p, "r", stdin)
#define wrf(p) freopen(p, "w", stdout)
#else
#define dbg
#define ast
#define rdf
#define wrf
#define dwr
#endif

char gc() { return getchar(); }
void pc(char c) { putchar(c); }
void el() { pc('\n'); }
void bp() { puts("Fuck ccf !"), fflush(stdout); }
void dtm(bool cond) { puts(cond ? "Yes" : "No"); }
void spc() { pc(' '); }
char rc() {
    auto av = [&](char c) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    };
    char c = gc();
    while (!av(c)) {
        c = gc();
    }
    return c;
}
void rs(string &s) {
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
int rd() {
    int ret = 0;
    bool sgn = 0;
    char c = gc();
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

template <int M> struct modint {
    int v;
    modint() : v(0) {}
    static modint uc(int v) {
        modint ret;
        ret.v = v;
        return ret;
    }
    explicit modint(int iv) : v((iv + M) % M) {}
    modint &operator+=(modint b) {
        v += b.v;
        if (v >= M) {
            v -= M;
        }
        return *this;
    }
    modint operator+(modint b) { return uc(v) += b; }
    modint &operator-=(modint b) {
        v -= b.v;
        if (v < 0) {
            v += M;
        }
        return *this;
    }
    modint operator-(modint b) { return uc(v) -= b; }
    modint &operator*=(modint b) {
        v = 1LL * v * b.v % M;
        return *this;
    }
    modint operator*(modint b) { return uc(v) *= b; }
    modint qpow(int b) {
        modint ret(1);
        for (; b; b >>= 1, *this *= *this) {
            if (b & 1) {
                ret *= *this;
            }
        }
        return ret;
    }
};
#define mint(mod) modint<mod>

void output(int x) { dwr("%lld", x); }
void output(sint x) { dwr("%d", x); }
template <int M> void output(mint(M) x) { dwr("%lld\n", x.v); }
void output(char c) { dwr("%c", c); }
void output(const char *s) { dwr("%s", s); }
template <typename A, typename B> void output(const p(A, B) & p) {
    pc('('), output(p.x), pc(','), output(p.y), pc(')');
}
template <typename C> void output(const C &v) {
    dwr("[");
    for (auto &elm : v) {
        output(elm), dwr(",");
    }
    dwr("]");
}

template <typename T> struct Ve {
    vector<T> v;
    bool empty() const { return size() > 0; }
    void resize(int n) { v.resize(n + 1); }
    void init(int n, const T &val) { v.clear(), v.resize(n + 1, val); }
    Ve() : v(vector<T>(1, T())) {}
    void reserve(int n) { v.reserve(n); }
    void clear() { v.resize(1); }
    int size() const { return siz(v) - 1; }
    const T &operator[](int idx) const {
        return ast(0 <= idx && idx <= size(), mp(idx, size())), v[idx];
    }
    T &operator[](int idx) {
        return ast(0 <= idx && idx <= size(), mp(idx, size())), v[idx];
    }
    using const_iter = typename ::vector<T>::const_iterator;
    using iter = typename ::vector<T>::iterator;
    const_iter begin() const { return v.begin() + 1; }
    iter begin() { return v.begin() + 1; }
    const_iter end() const { return v.end(); }
    iter end() { return v.end(); }
    const T &front() const { return ast(size() > 0, "front"), v[1]; }
    const T &back() const { return ast(size() > 0, "back"), v.back(); }
    T &front() { return ast(size() > 0, "front"), v[1]; }
    T &back() { return ast(size() > 0, "back"), v.back(); }
    void push(const T &val) { v.push_back(val); }
    void pop() { ast(size() > 0, "pop"), v.pop_back(); }
    void erase(int l, int r) { v.erase(v.begin() + l, v.begin() + r + 1); }
    void insert(int at, const T &val) { v.insert(v.begin() + at, val); }
};

void updmax(int &x, int y) { x = max(x, y); }
void updmin(int &x, int y) { x = min(x, y); }
int gcd(int x, int y) { return __gcd(x, y); }
int lcm(int x, int y) { return x / gcd(x, y) * y; }
int lowbit(int x) { return x & (-x); }
int max(int x, int y) { return x > y ? x : y; }
int min(int x, int y) { return x < y ? x : y; }
int sqr(int x) { return x * x; }
int ceil(int x, int y) { return (x + y - 1) / y; }

constexpr int mod = 1e9 + 7;
// constexpr int mod = 998244353;
typedef mint(mod) mint;

constexpr int N = -1;
constexpr int inf = 0x3f3f3f3f3f3f3f3f;

void task() {}

sint main() {
    // int T = read();
    int T = 1;
    while (T--) {
        task();
    }
    return 0;
}
