#include <bits/stdc++.h>
#include <cstdio>
using namespace std;

#define int long long
// #define double long double
#define siz(x) ((int)(x).size())
#define me(a, v) memset(a, v, sizeof(a))
#define L(i, l, r) for (int i = (l); i <= (r); i++)
#define R(i, r, l) for (int i = (r); i >= (l); i--)
#define x first
#define y second

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

template <typename T> struct Ve {
    vector<T> v;
    void init(int n, const T &val) {
        v.clear();
        v.resize(n + 1, val);
    }
    Ve() : v(vector<T>(1, T())) {}
    Ve(int n, const T &val) { init(n, val); }
    void reserve(int n) { v.reserve(n); }
    void clear() { v.resize(1); }
    int size() const { return siz(v) - 1; }
    void check_empty(const char *msg) const {
#ifdef DEBUG
        if (siz(v) <= 1) {
            printf("Panic : %s vector is empty !", msg);
            exit(823543);
        }
#endif
    }
    void check_index(const char *msg, int idx) const {
#ifdef DEBUG
        if (!(idx >= 0 && idx <= size())) {
            printf("Panic : %s index %lld out of bound %lld\n", msg, idx,
                   size());
            exit(823543);
        }
#endif
    }
    const T &operator[](int idx) const {
        check_index("[]", idx);
        return v[idx];
    }
    T &operator[](int idx) {
        check_index("[]", idx);
        return v[idx];
    }
    const T *begin() const { return v.begin() + 1; }
    T *begin() { return v.begin() + 1; }
    const T *end() const { return v.end(); }
    T *end() { return v.end(); }
    const T &front() const {
        check_empty("front");
        return v[1];
    }
    const T &back() const {
        check_empty("back");
        return v.back();
    }
    void push(const T &val) { v.push_back(val); }
    void pop() {
        check_empty("pop");
        assert(size() > 0);
        v.pop_back();
    }
    void erase(int l, int r) { v.erase(v.begin() + l, v.begin() + r + 1); }
    void insert(int at, const T &val) { v.insert(v.begin() + at, val); }
};

template <typename T, int N> struct Ar {
    T v[N + 1];
    int n;
    void init(int in, const T &val) {
        n = in;
        L(i, 0, n) { v[i] = val; }
    }
    Ar() : v(vector<T>(1, T())) {}
    Ar(int n, const T &val) { init(n, val); }
    void clear() { n = 0; }
    int size() const { return n; }
    void check_empty(const char *msg) const {
#ifdef DEBUG
        if (siz(v) <= 1) {
            printf("Panic : %s vector is empty !", msg);
            exit(823543);
        }
#endif
    }
    void check_index(const char *msg, int idx) const {
#ifdef DEBUG
        if (!(idx >= 0 && idx <= size())) {
            printf("Panic : %s index %lld out of bound %lld\n", msg, idx,
                   size());
            exit(823543);
        }
#endif
    }
    const T &operator[](int idx) const {
        check_index("[]", idx);
        return v[idx];
    }
    T &operator[](int idx) {
        check_index("[]", idx);
        return v[idx];
    }
    const T *begin() const { return v.begin() + 1; }
    T *begin() { return v.begin() + 1; }
    const T *end() const { return v.end(); }
    T *end() { return v.end(); }

    const T &front() const {
        check_empty("front");
        return v[1];
    }
    const T &back() const {
        check_empty("back");
        return v.back();
    }
    void push(const T &val) { v[++n] = val; }
    void pop() {
        check_empty("pop");
        assert(size() > 0);
        n--;
    }
    void erase(int l, int r) { v.erase(v.begin() + l, v.begin() + r + 1); }
    void insert(int at, const T &val) { v.insert(v.begin() + at, val); }
};

#define ve(a) Ve<a>
#define sve(a) static Ve<a>
#define ar(a, n) Ar<a, n>
#define sar(a, n) static Ar<a, n>
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

#define output_collection(c)                                                   \
    template <typename A, typename B> void output(const c &v) {                \
        printf("[");                                                           \
        for (auto &elm : v) {                                                  \
            output(elm), printf(",");                                          \
        }                                                                      \
        printf("]");                                                           \
    }
void output(int x) { printf("%lld", x); }
template <int M> void output(mint(M) x) { printf("%lld\n", x.v); }
void output(char c) { printf("%c", c); }
void output(const char *s) { printf("%s", s); }
template <typename A, typename B> void output(const p(A, B) & p) {
    pc('('), output(p.x), pc(','), output(p.y), pc(')');
}

output_collection(ve(A));
output_collection(set(A));
output_collection(map(A, B));
output_collection(mset(A));
output_collection(mmap(A, B));
output_collection(hset(A));
output_collection(hmap(A, B));
output_collection(hmset(A));
output_collection(hmmap(A, B));

#define dbg(x) printf("%s = ", #x), output(x), el()

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
    int T = qread();
    while (T--) {
        task();
    }
    return 0;
}
