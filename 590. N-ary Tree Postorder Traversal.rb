def postorder(root)
  return [] unless root

  root.children.flat_map do |child|
    postorder(child)
  end + [root.val]
end
