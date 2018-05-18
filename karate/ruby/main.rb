require "test/unit/assertions"
require "benchmark"
include Test::Unit::Assertions

def chop(int, array)
  array.each_index do |index|
    if array[index] == int
      return index
    end
  end
  return -1
end

def binary_chop(int, array)
  half = array.length / 2
  thread_1 = Thread.new { search_array(int, array[0..half]) }
  thread_2 = nil
  if half != 0
    thread_2 = Thread.new { search_array(int, array[half..array.length], half) }
  end

  if thread_1.value != -1
    val = thread_1.value
    thread_1.kill
    thread_2.kill if thread_2
    return val
  elsif thread_2 && thread_2.value != -1
    val = thread_2.value
    thread_1.kill
    thread_2.kill
    return val
  else
    thread_1.kill
    thread_2.kill if thread_2
    return -1
  end
end

def search_array(int, array, add_by=0)
  array.each_index do |index|
    if array[index] == int
      return index + add_by
    end
  end
  return -1
end

def test_chop
  assert_equal(-1, chop(3, []))
  assert_equal(-1, chop(3, [1]))
  assert_equal(0,  chop(1, [1]))
  #
  assert_equal(0,  chop(1, [1, 3, 5]))
  assert_equal(1,  chop(3, [1, 3, 5]))
  assert_equal(2,  chop(5, [1, 3, 5]))
  assert_equal(-1, chop(0, [1, 3, 5]))
  assert_equal(-1, chop(2, [1, 3, 5]))
  assert_equal(-1, chop(4, [1, 3, 5]))
  assert_equal(-1, chop(6, [1, 3, 5]))
  #
  assert_equal(0,  chop(1, [1, 3, 5, 7]))
  assert_equal(1,  chop(3, [1, 3, 5, 7]))
  assert_equal(2,  chop(5, [1, 3, 5, 7]))
  assert_equal(3,  chop(7, [1, 3, 5, 7]))
  assert_equal(-1, chop(0, [1, 3, 5, 7]))
  assert_equal(-1, chop(2, [1, 3, 5, 7]))
  assert_equal(-1, chop(4, [1, 3, 5, 7]))
  assert_equal(-1, chop(6, [1, 3, 5, 7]))
  assert_equal(-1, chop(8, [1, 3, 5, 7]))

  big_array = []
  count = 0
  15000.times do
    big_array.push(count)
    count = count + 1
  end

  assert_equal(13500, chop(13500, big_array))
end

def test_binary_chop
  assert_equal(-1, binary_chop(3, []))
  assert_equal(-1, binary_chop(3, [1]))
  assert_equal(0,  binary_chop(1, [1]))

  assert_equal(0,  binary_chop(1, [1, 3, 5]))
  assert_equal(1,  binary_chop(3, [1, 3, 5]))
  assert_equal(2,  binary_chop(5, [1, 3, 5]))
  assert_equal(-1, binary_chop(0, [1, 3, 5]))
  assert_equal(-1, binary_chop(2, [1, 3, 5]))
  assert_equal(-1, binary_chop(4, [1, 3, 5]))
  assert_equal(-1, binary_chop(6, [1, 3, 5]))
  #
  assert_equal(0,  binary_chop(1, [1, 3, 5, 7]))
  assert_equal(1,  binary_chop(3, [1, 3, 5, 7]))
  assert_equal(2,  binary_chop(5, [1, 3, 5, 7]))
  assert_equal(3,  binary_chop(7, [1, 3, 5, 7]))
  assert_equal(-1, binary_chop(0, [1, 3, 5, 7]))
  assert_equal(-1, binary_chop(2, [1, 3, 5, 7]))
  assert_equal(-1, binary_chop(4, [1, 3, 5, 7]))
  assert_equal(-1, binary_chop(6, [1, 3, 5, 7]))
  assert_equal(-1, binary_chop(8, [1, 3, 5, 7]))

  big_array = []
  count = 0
  15000.times do
    big_array.push(count)
    count = count + 1
  end

  assert_equal(13500, binary_chop(13500, big_array))
end

Benchmark.bm do |benchmark|
  benchmark.report("Chop") do
    test_chop
  end
  benchmark.report("Bhop") do
    test_binary_chop
  end
end
