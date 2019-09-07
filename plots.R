library(tidyverse)
data <- read_tsv("out.txt")
split <- gather(data, "stat", "value", -name, -b, -s) %>% filter(stat != "stdev")
ggplot(split) +
  geom_point(aes(x = b, y = value, color = name, shape = name)) +
  facet_wrap(~stat) + labs(
    title = "Size of GCD with others",
    y = "GCD Length (bits)",
    x = "Output length (bits)",
    color = "PRG",
    stat = "Statistic"
  ) + scale_color_discrete(
    name="PRG",
    breaks=c("inc", "uniform"),
    labels=c("128b uniform, extended with increment", "fully uniform")
  )
