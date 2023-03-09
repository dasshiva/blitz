func main
	mov r29, sp
	mov r0, 1
	mov r1, 2
	add r0, r0, r1
	lea r1, [r29 + 4]
	mov [r1], r0
	mov sp, r29
end
