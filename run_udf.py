import polars as pl
from capped import capped_cum_sum

df = pl.DataFrame({
    "values": [1,0,1,-1,-1,0,0,0,-1,1,1,1,0,0,0]
})
result = df.with_columns(result=capped_cum_sum('values',cap=12))

print(result)
