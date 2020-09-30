[StructLayout(LayoutKind.Sequential)]
public struct RustBuffer<T> where T : unmanaged
{
    private readonly UInt32 capacity;
    private readonly UInt32 len;
    private readonly unsafe void* data;
}
