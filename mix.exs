defmodule Collections.MixProject do
  use Mix.Project

  def project do
    [
      app: :collections,
      version: "0.1.0",
      elixir: "~> 1.8",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.16.0"},
      {:benchee, "~> 0.9", only: :dev}
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end

  defp rustler_crates do
    [
      collections_algorithms: [path: "native/collections_algorithms", mode: if(Mix.env() == :prod, do: :release, else: :debug)],
      collections_datatypes: [path: "native/collections_datatypes", mode: if(Mix.env() == :prod, do: :release, else: :debug)]
    ]
  end
end
