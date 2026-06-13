const POW2:[i32;100005]={
    let mut arr=[0;100005];
    arr[0]=1;
    let mut i=1;
    while i<100005{
        arr[i]=((arr[i-1]as i64 *2)%1_000_000_007) as i32;
        i+=1;
    }
    arr
};
impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let  n=edges.len()+1;

    let mut arena=vec![0u32;6*n+1];//[arena(6*n+1)]
        let (front,back)=arena.split_at_mut(3*n+1);//[front(3n+1)|back(3n)]
        let (depth,rest)=back.split_at_mut(n);//[front(3n+1)|depth(n)|rest(2n)]
        let (parent,queue)=rest.split_at_mut(n);//[front(3n+1)|depth(n)|parent(n)|queue(n)]

        {
            let (head,links)=front.split_at_mut(n+1);
            //[head(n+1)|links(2n)|depth(n)|parent(n)|queue(n)]

            for edge in &edges {
                head[(edge[0]-1)as usize]+=1;
                head[(edge[1]-1)as usize]+=1;
            }

            for i in 0..n{
                let prev=head[i];
                head[i+1]+=prev;
            }

            for edge in &edges{
                let u=(edge[0]-1)as usize;
                let v=(edge[1]-1)as usize;

                head[u]-=1;
                links[head[u]as usize]=v as u32;

                head[v]-=1;
                links[head[v]as usize]=u as u32;

            }
            drop(edges);

            depth[0]=0;
            parent[0]=n as u32;
            queue[0]=0;

            let mut queue_begin=0;
            let mut queue_end=1;
            while queue_begin !=queue_end{
                let u=queue[queue_begin] as usize;
                queue_begin+=1;

                let start=head[u] as usize;
                let end=head[u+1] as usize;
                for i in start..end{
                    let v=links[i]as usize;
                    if v as u32 !=parent[u]{
                       parent[v] =u as u32;

                        let depth_u=depth[u];
                        depth[v]=depth_u+1;

                        queue[queue_end]=v as u32;
                        queue_end+=1;
                    }
                }
            }
        }
        {
            let (sz,heavy)=front.split_at_mut(n+1);
            //[sz(n+1)|heavy(2n)|depth(n)|parent(n)|queue(n)]

            for i in 0..n{
                sz[i]=1;
                heavy[i]=n as u32;
            }

            for i in (1..n).rev(){
                let u=queue[i] as usize;
                let p=parent[u] as usize;

                let sz_u=sz[u];
                sz[p]+=sz_u;

                let hp=heavy[p]as usize;
                if hp==n || sz_u>sz[hp]{
                    heavy[p]=u as u32;
                }
            }
        }

        let (hld_head,heavy)=front.split_at_mut(n+1);

        for i in 0..n{
            let u=queue[i]as usize;
            let pu=parent[u] as usize;
            if u==0 || heavy[pu]as usize!=u{
                hld_head[u]=u as u32;
            }else{
                let hh_pu=hld_head[pu];
                hld_head[u]=hh_pu;
            }
        }

        let hld_head:&[u32]=hld_head;
        let depth:&[u32]=depth;
        let parent:&[u32]=parent;

        let get_lca=move|mut u:usize,mut v:usize|->usize{
            while hld_head[u]!=hld_head[v]{
                let hu=hld_head[u]as usize;
                let hv=hld_head[v]as usize;
                if depth[hu]>depth[hv]{
                    u=parent[hu] as usize;
                }else{
                    v=parent[hv] as usize;
                }
            }
            if depth[u]<depth[v]{u}else{v}
        };

        queries.into_iter().map(|q|{
            unsafe{
                let u=(*q.get_unchecked(0)-1)as usize;
                let v=(*q.get_unchecked(1)-1)as usize;

                if u==v{
                    0
                }else{
                    let lca=get_lca(u,v);
                    let dist=*depth.get_unchecked(u)+*depth.get_unchecked(v)-2* *depth.get_unchecked(lca);
                    *POW2.get_unchecked((dist-1)as usize)
                }
            }
        }).collect()
    }
          }
