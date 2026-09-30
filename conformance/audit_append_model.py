#!/usr/bin/env python3
from dataclasses import dataclass
from collections import deque
@dataclass(frozen=True)
class S: rows:int=0; committed:int=0
def nxt(s):
    out=[]
    if s.rows<2: out.append(S(s.rows+1,s.committed))
    if s.committed<s.rows: out.append(S(s.rows,s.rows))
    return out
def main():
    q=deque([S()]); seen={S()}; edges=0
    while q:
        s=q.popleft(); assert 0<=s.committed<=s.rows
        for n in nxt(s):
            edges+=1; assert n.rows>=s.rows,'audit rows deleted'; assert n.committed>=s.committed,'commit watermark regressed'
            if n not in seen: seen.add(n); q.append(n)
    assert S(2,2) in seen
    print(f'audit append model: {len(seen)} states, {edges} transitions')
if __name__=='__main__': main()
