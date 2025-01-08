import polars as pl
from capped import capped_cum_sum as custom_capped_cum_sum

@pl.api.register_expr_namespace("lmt")
class bounds:
    def __init__(self, expr: pl.Expr) -> None:
        self._expr = expr

    def capped_cum_sum(self,cap) -> pl.Expr:
        return custom_capped_cum_sum(self._expr,cap=cap)


df = pl.DataFrame({
    "values": [5,5,5,12,-16,-3,4,5,6,-10]
})
# result = df.with_columns(result=capped_cum_sum('values',cap=12))
result = df.with_columns(
    pl.col('values').lmt.capped_cum_sum(cap=12).alias('result')
)

print(result)
