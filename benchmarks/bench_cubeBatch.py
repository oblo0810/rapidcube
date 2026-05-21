import rapidcube


def test_benchmark_get_next_states(benchmark):
    cubes = rapidcube.CubeBatch(10000, 2)
    benchmark(cubes.get_next_states)
