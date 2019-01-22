defmodule CollectionsTest do
  use ExUnit.Case
  doctest Collections

  test "greets the world" do
    assert Collections.hello() == :world
  end
end


defmodule Collections.AlgorithmsTest do
  use ExUnit.Case
  doctest Collections.Algorithms

  test "check it works" do
    assert Collections.Algorithms.add(12, 2) == {:ok, 14}
  end

  test"check list works" do
    assert Collections.Algorithms.list([1, 2, 3]) === {:ok, [1, 2, 3]}
  end
end


defmodule Collections.DatatypesTest do
  use ExUnit.Case
  doctest Collections.Datatypes

  test "check it works" do
    assert Collections.Datatypes.add(12, 2) == {:ok, 14}
  end
end