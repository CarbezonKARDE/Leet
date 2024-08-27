require 'set'
class Array
    def add_edge(a, b, w)
        self[a] << [b, w]
        self[b] << [a, w]
    end
end
class Solver
    attr_reader :n, :edges, :succ_prob, :neighbs
    def initialize(n, edges, succ_prob)
        @n, @edges, @succ_prob = n, edges, succ_prob
        @neighbs = edges.each_with_index.each_with_object(Array.new(n) {|_| Array.new }) {|((a, b), idx), res| 
            res.add_edge(a, b, succ_prob[idx])
        }
    end
    def solve(s, e)
        (dp = Array.new(n))[s] = 1.0
        cur = [s]
        until cur.empty? do
            cur = cur.inject(Set[]) {|res, c|
                res.merge(neighbs[c].filter {|(ne, w)| 
                        dp[ne].nil? || dp[c] * w > dp[ne] 
                    }.collect {|(ne, w)|
                            dp[ne] = dp[c] * w
                            ne
                    })
            }
        end
        dp[e]
    end
end
def max_probability(n, edges, succ_prob, s, e)
    Solver.new(n, edges, succ_prob).solve(s, e) || 0
end
