defmodule Collections do
  @moduledoc """
  Documentation for Collections.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Collections.hello()
      :world

  """
  def hello do
    :world
  end
end

defmodule Collections.Algorithms do
  use Rustler, otp_app: :collections, crate: "collections_algorithms"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end

defmodule Collections.Datatypes do
  use Rustler, otp_app: :collections, crate: "collections_datatypes"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end