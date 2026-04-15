from rapidcube import Cube2x2, Cube3x3
import torch


def main():
    cube = Cube2x2()
    # print(cube)
    # cube.do_u_prime_move()
    # print(cube)
    # cube.do_d_move()
    # print(cube)
    # cube.do_r_prime_move()
    # print(cube)
    # cube.do_l_move()
    # print(cube)
    # cube.do_moves("R U R' U' R' F R2 U' R' U' R U R' F'")
    # print(cube)
    # cube.do_moves("R U R' U' R' F R2 U' R' U' R U R' F'")
    cube.do_moves("D")
    print(cube)
    cube.do_moves("D'")
    print(cube)


def test_3x3():
    cube = Cube3x3()
    # print(cube)
    # cube.do_u_prime_move()
    # print(cube)
    # cube.do_d_move()
    # print(cube)
    # cube.do_u_move()
    # print(cube)
    # cube.do_d_prime_move()
    # print(cube)
    # cube.do_b_move()
    # print(cube)
    # cube.do_b_prime_move()
    # print(cube)
    # cube.do_moves("R U R' U' R' F R2 U' R' U' R U R' F'")
    # print(cube)
    # cube.do_moves("R U R' U' R' F R2 U' R' U' R U R' F'")
    # print(cube)
    # moves = "F' R F R2 B' D2 R2 L F' L2 D2 L2 D L2 F2 U R2 D L2 B2 D"
    # for m in moves.split(" "):
    #     print("Doing Move: " + m + "\n")
    #     cube.do_moves(m)
    #     print(cube)
    cube.do_moves("R")
    print(cube)


def encode_state():
    cube = Cube2x2()
    str_state = "00" + "{0:b}".format(cube.state)
    segments = [
        str_state[max(0, end - 5) : end] for end in range(len(str_state), 0, -5)
    ][::-1]
    int_segments = [[int(s[:2], 2), int(s[2:], 2)] for s in segments]

    tensor = torch.tensor(int_segments)
    # print(str_state)
    # print(segments)
    # print(tensor)
    return tensor


if __name__ == "__main__":
    _ = encode_state()
    test_3x3()
