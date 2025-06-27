# Physics Performance Optimization Plan

Based on benchmark analysis showing 35-45% of physics time spent on grid queries and memory allocation overhead, here's a comprehensive optimization strategy:

## Phase 1: Grid Query Optimization (High Impact - 35-45% performance gain)

### 1.1 Eliminate Vec Collection Overhead
**Problem**: Current grid neighbor lookup uses `.collect::<Vec<_>>()` which allocates memory on every physics frame
**Solution**: Replace with direct iterator processing or pre-allocated buffer
- Modify `get_neighbors` to work with iterators directly
- Add iterator-based collision checking to avoid allocation
- Expected gain: 1,500-2,000 cycles per frame

### 1.2 Optimize Grid Cell Iteration
**Problem**: Creates 9-element array and filters, generating intermediate collections
**Solution**: Direct cell lookup with early termination
- Replace array iteration with direct HashMap lookups
- Skip empty cells immediately  
- Use inline functions for hot path code

## Phase 2: Memory Allocation Reduction (Medium Impact)

### 2.1 Pre-allocated Neighbor Buffers
**Problem**: Dynamic allocation in collision detection hot path
**Solution**: Add fixed-size neighbor buffer to PhysicsState
- Pre-allocate `[usize; MAX_NEIGHBORS]` buffer (typically 9-16 pegs max)
- Reuse buffer across frames to eliminate allocations
- Pass buffer slice to collision functions

### 2.2 Stack-based Grid Queries  
**Problem**: HashMap and Vec allocations in grid operations
**Solution**: Stack-allocated arrays for common cases
- Use `[usize; 16]` for typical neighbor counts
- Fallback to Vec only for dense scenarios (rare)

## Phase 3: Algorithm Micro-optimizations (Low-Medium Impact)

### 3.1 Grid Coordinate Optimizations
**Problem**: Repeated coordinate transformations and bounds checking
**Solution**: Cached coordinate mapping and bounds pre-checks
- Cache grid coordinates for ball position
- Pre-compute valid grid bounds to avoid Option handling
- Use bit operations for coordinate math where possible

### 3.2 Collision Detection Pipeline
**Problem**: Multiple closure calls and conditional compilation overhead
**Solution**: Unified hot path with compile-time specialization  
- Create single optimized collision function
- Remove benchmark overhead from production code paths
- Inline critical collision math

## Phase 4: Data Structure Optimizations (Low Impact)

### 4.1 Spatial Grid Improvements
**Problem**: HashMap overhead for small cell counts
**Solution**: Hybrid storage for sparse vs dense scenarios
- Direct array for common grid cells (center area)
- HashMap fallback for edge cases
- More cache-friendly memory layout

### 4.2 Peg Data Layout
**Problem**: Multiple array accesses for peg properties
**Solution**: Structure-of-Arrays to Array-of-Structures conversion
- Group frequently accessed peg data together
- Reduce cache misses in collision loops
- Optimize for GBA's memory hierarchy

## Implementation Priority & Expected Gains

### Phase 1 (Must-Do): Grid Query Optimization
- **Effort**: 2-3 hours implementation + testing
- **Impact**: 30-40% physics performance improvement  
- **Risk**: Low - well-understood optimization
- **Target**: Reduce grid query from ~2,000 to ~500-800 cycles

### Phase 2 (Should-Do): Memory Allocation Reduction  
- **Effort**: 1-2 hours implementation
- **Impact**: 10-15% additional improvement
- **Risk**: Low - standard embedded optimization
- **Target**: Eliminate all per-frame allocations

### Phase 3 (Nice-to-Have): Algorithm Micro-optimizations
- **Effort**: 3-4 hours implementation + benchmarking
- **Impact**: 5-10% additional improvement  
- **Risk**: Medium - needs careful benchmarking
- **Target**: Optimize hot path instruction counts

### Phase 4 (Future): Data Structure Optimization
- **Effort**: 4-6 hours major refactoring
- **Impact**: 5-10% improvement, better scalability
- **Risk**: High - significant code changes
- **Target**: Better cache utilization and scalability

## Success Metrics

- **Target 1**: Reduce total physics time from ~7,000 to ~4,500 cycles (35% improvement)
- **Target 2**: Grid query time from ~2,000 to ~500 cycles (75% improvement)  
- **Target 3**: Maintain collision detection at <200 cycles (current efficiency)
- **Target 4**: Physics budget reduced from 3% to 2% of frame time

## Validation Strategy

1. **Benchmark each phase**: Measure before/after performance 
2. **Regression testing**: Ensure physics behavior unchanged
3. **Stress testing**: Verify improvements under maximum peg density
4. **Memory profiling**: Confirm elimination of allocations

This optimization plan targets the root cause (Vec allocation overhead) while providing incremental improvements that can be implemented and validated independently.