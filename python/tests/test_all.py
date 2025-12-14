import pytest
import kit_opt


def test_sum_as_string():
    assert kit_opt.sum_as_string(1, 1) == "2"
