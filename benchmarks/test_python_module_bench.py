import fastcube

Cube2x2 = fastcube.Cube2x2


def test_benchmark_construct_cube(benchmark):
    benchmark(Cube2x2)


def test_benchmark_single_move_dispatch(benchmark):
    cube = Cube2x2()
    benchmark(cube.do_r_move)


def test_benchmark_state_property_read(benchmark):
    cube = Cube2x2()
    benchmark(lambda: cube.state)


def test_benchmark_to_binary(benchmark):
    cube = Cube2x2()
    benchmark(cube.to_binary)


def test_benchmark_str_render(benchmark):
    cube = Cube2x2()
    benchmark(str, cube)


def test_benchmark_short_algorithm_on_same_cube(benchmark):
    cube = Cube2x2()

    def run_alg():
        cube.do_r_move()
        cube.do_u_move()
        cube.do_f_prime_move()
        cube.do_u_prime_move()
        cube.do_r_prime_move()

    benchmark(run_alg)
