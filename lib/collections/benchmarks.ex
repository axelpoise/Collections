defmodule Collections.Benchmarks do
  @moduledoc false

  alias Collections.Algorithms
  Benchee.run(
    %{
      "insertion_sort" => fn -> Algorithms.insertion_sort(Enum.to_list(1..10_000)) end
    }
  )
end
