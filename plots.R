library(tidyverse)
data <- read_tsv("plus-prime.txt")
split <- gather(data, "stat", "value", -name, -b, -s) %>% filter(stat != "stdev")
ggplot(split) +
  geom_point(aes(x = b, y = value, color = name)) +
  facet_wrap(~stat) + labs(
    title = "Size of GCD with others",
    y = "GCD Length (bits)",
    x = "Output length (bits)",
    color = "PRG",
    stat = "Statistic"
  ) + scale_color_discrete(
    name="PRG",
    breaks=c("inc", "uniform", "plus-prime"),
    labels=c("128b uniform, extended with increment", "fully uniform", "plus a prime")
  )

ggsave("plot.png", width = 8, height = 5, units = "in")
