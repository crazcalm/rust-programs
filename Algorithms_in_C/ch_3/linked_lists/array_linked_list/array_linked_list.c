/*
  The original code used `max+2` and not 12.

  I changed it because it was giving me compile errors.
 */
int key[12], next[12];
int x, head, z;

int listinitialize()
{
  head = 0; z = 1; x = 2;
  next[head] = z; next[z] = z;
 };
int deletenext(int t)
  { next[t] = next[next[t]]; };
int insertafter(int v, int t)
{
  key[x] = v; next[x] = next[t];
  next[t] = x;
  return x++;
};

int main(){
}
