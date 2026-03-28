from fastcube import Cube2x2


def main():
    cube = Cube2x2()
    print(cube)
    cube.do_u_prime_move()
    print(cube)
    cube.do_d_move()
    print(cube)
    cube.do_r_prime_move()
    print(cube)
    cube.do_l_move()
    print(cube)


if __name__ == "__main__":
    main()
