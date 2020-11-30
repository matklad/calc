#include "stdio.h"

int ccacl_eval(char const *expr, long long const *res);

int main(void) {
  long long value;
  int ret = ccacl_eval("3 sqr 4 sqr +", &value);
  if (ret == 0) {
    printf("%lld\n", value);
  }
  return 0;
}
