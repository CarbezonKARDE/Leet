#include <ranges>
class Solution {
public:
  static auto largestRectangleArea(const std::span<const int> heights) -> int {
    const auto n = static_cast<int>(std::ranges::size(heights));
    auto stack = std::vector<int>();
    auto ans = 0;
    for (const auto i : std::views::iota(0, n + 1)) {
      while (not std::ranges::empty(stack) and
             (i == n or heights[i] < heights[stack.back()])) {
        const auto height = heights[stack.back()];
        stack.pop_back();
        const auto width = std::ranges::empty(stack) ? i : i - stack.back() - 1;
        ans = std::max(ans, height * width);
      }
      stack.push_back(i);
    }
    return ans;
  }
};
